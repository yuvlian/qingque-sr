import os
import subprocess

files = ["sqlite.db", "sqlite.db-shm", "sqlite.db-wal"]

def delete_sqlite_files():
    for file in files:
        try:
            os.remove(file)
            print(f"Deleted {file}")
        except FileNotFoundError:
            print(f"{file} not found, skipping.")
        except Exception as e:
            print(f"Error deleting {file}: {e}")

def run_sqlx_commands():
    try:
        subprocess.run(["sqlx", "database", "create"], check=True)
        subprocess.run(["sqlx", "migrate", "run"], check=True)
    except subprocess.CalledProcessError as e:
        print(f"Error running sqlx command: {e}")

def main():
    delete_sqlite_files()
    run_sqlx_commands()

if __name__ == "__main__":
    main()
