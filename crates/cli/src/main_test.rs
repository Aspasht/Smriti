#[cfg(test)]
mod tests {
    use crate::{CliArgs, SmritiCli};
    use clap::Parser;
    use database::*;
    use rusqlite::{Connection, Result};

    pub fn db_connect() -> Result<Connection> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS commands (
                 id INTEGER PRIMARY KEY,
                 command TEXT NOT NULL UNIQUE,
                 alias TEXT UNIQUE,
                 info TEXT,
                 service TEXT
            )",
            [],
        )?;
        Ok(conn) // Return the connection
    }

    const ALIAS: &str = "init_alias";
    const NEW_ALIAS: &str = "new_alias";
    const COMMAND: &str = "echo hello";
    // const NEW_COMMAND: &str = "echo world";
    const SERVICE: &str = "test_service";
    const INFO: &str = "test_info";

    #[test]
    fn test_rename_alias() -> Result<()> {
        let conn = db_connect()?;

        let matches = SmritiCli::parse_from(&["smriti", "rename", ALIAS, NEW_ALIAS]);
        if let CliArgs::Rename(rename) = matches.command {
            assert_eq!(rename.alias, ALIAS);
            assert_eq!(rename.new_alias, NEW_ALIAS);
        } else {
            panic!("Expected Rename command");
        }

        insert_command(&conn, COMMAND, ALIAS, INFO, SERVICE)?;
        rename_alias(&conn, ALIAS, NEW_ALIAS)?;
        let retrieved_command = retrieve_command_by_alias(&conn, NEW_ALIAS)?;
        assert_eq!(retrieved_command.alias, NEW_ALIAS);

        Ok(())
    }

    #[test]
    fn test_delete_by_alias() -> Result<()> {
        let conn = db_connect()?;
        insert_command(&conn, COMMAND, ALIAS, INFO, SERVICE)?;

        // Now, test deleting the alias
        let matches = SmritiCli::parse_from(&["smriti", "delete", "-a", ALIAS]);

        if let CliArgs::Delete(_delete) = matches.command {
            delete_by_alias(&conn, ALIAS)?;
            let result = retrieve_command_by_alias(&conn, ALIAS);
            assert!(
                result.is_err(),
                "Command should be deleted and not retrievable"
            );
        } else {
            panic!("Expected Delete command");
        }
        Ok(())
    }

    #[test]
    fn test_delete_by_service() -> Result<()> {
        let conn = db_connect()?;
        insert_command(&conn, COMMAND, ALIAS, INFO, SERVICE)?;

        let matches = SmritiCli::parse_from(&["smriti", "delete", "-s", SERVICE]);

        if let CliArgs::Delete(_delete) = matches.command {
            delete_by_service(&conn, SERVICE)?;
            let result = retrieve_commands_by_service(&conn, SERVICE)?;
            assert!(
                result.is_empty(),
                "Commands should be deleted and not retrievable, but found some"
            );
        } else {
            panic!("Expected Delete command");
        }
        Ok(())
    }
}
