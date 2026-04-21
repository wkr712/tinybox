import Database from "@tauri-apps/plugin-sql";

let db: Database | null = null;

export async function getDb(): Promise<Database> {
  if (!db) {
    db = await Database.load("sqlite:tinybox.db");
  }
  return db;
}

export async function select<T>(query: string, params?: unknown[]): Promise<T[]> {
  const database = await getDb();
  return database.select<T[]>(query, params);
}

export async function execute(query: string, params?: unknown[]): Promise<void> {
  const database = await getDb();
  await database.execute(query, params);
}
