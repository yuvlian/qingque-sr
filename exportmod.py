import argparse
import os

def generate_mod_rs(directory: str):
    mod_file = os.path.join(directory, "mod.rs")

    modules = [
        os.path.splitext(f)[0] for f in os.listdir(directory)
        if f.endswith(".rs") and f != "mod.rs"
    ]
    
    modules.sort()
    
    with open(mod_file, "w", encoding="utf-8") as f:
        for module in modules:
            f.write(f"pub mod {module};\n")
    
    print(f"Updated {mod_file} with {len(modules)} modules.")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate mod.rs with modules in same dir.")
    parser.add_argument("directory", type=str, help="Directory containing Rust files.")
    args = parser.parse_args()
    
    generate_mod_rs(args.directory)
