pub mod prod {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("seed/prod");

    pub fn run(
        connection: &mut impl diesel_migrations::MigrationHarness<diesel::pg::Pg>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let res = connection.run_pending_migrations(MIGRATIONS);
        match res {
            Err(err) => {
                error!("Error failed to seed with profile prod: {:?}", err);
                Err(err)
            }
            Ok(res) => {
                if res.is_empty() {
                    info!("Sucessfully seeded with profile prod");
                }
                Ok(())
            }
        }
    }
}

pub mod dev {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("seed/dev");

    pub fn run(
        connection: &mut impl diesel_migrations::MigrationHarness<diesel::pg::Pg>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let res = connection.run_pending_migrations(MIGRATIONS);
        match res {
            Err(err) => {
                error!("Error failed to seed with profile dev: {:?}", err);
                Err(err)
            }
            Ok(res) => {
                if res.is_empty() {
                    info!("Sucessfully seeded with profile dev");
                }
                Ok(())
            }
        }
    }
}

pub mod test {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("seed/test");

    pub fn run(
        connection: &mut impl diesel_migrations::MigrationHarness<diesel::pg::Pg>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let res = connection.run_pending_migrations(MIGRATIONS);
        match res {
            Err(err) => {
                error!("Error failed to seed with profile test: {:?}", err);
                Err(err)
            }
            Ok(res) => {
                if res.is_empty() {
                    info!("Sucessfully seeded with profile test");
                }
                Ok(())
            }
        }
    }
}
