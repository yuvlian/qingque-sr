package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"
)

func checkFileExists(filePath string) bool {
	_, err := os.Stat(filePath)
	return os.IsNotExist(err)
}

func checkToolExists(tool string) bool {
	if _, err := exec.LookPath(tool); err != nil {
		return false
	}
	return true
}

func compileProto(filePath string) {
	log.Printf("running compileProto(\"%s\").\n", filePath)

	compiledPath := strings.Replace(filePath, ".proto", ".pb.go", 1)

	if !checkFileExists(compiledPath) {
		log.Printf("\"%s\" is already compiled. skipping.", filePath)
	}

	if !checkToolExists("protoc") {
		log.Fatalln("'protoc' not found. get it from https://github.com/protocolbuffers/protobuf/releases/tag/v31.1")
	}

	if !checkToolExists("protoc-gen-go") {
		log.Fatalln("'protoc-gen-go' not found. get it with: go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.36.6")
	}

	cmd := exec.Command("protoc", "--go_out=.", filePath)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Run(); err != nil {
		log.Fatalf("compileProto failed: %v\n", err)
	} else {
		log.Println("compileProto success.")
		log.Println("extra: generating cid file.")
	}

	data, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatalf("failed to read proto file: %v", err)
	}

	re := regexp.MustCompile(`\sCmd(\w*)\s*=\s*(\d+)`)
	matches := re.FindAllStringSubmatch(string(data), -1)

	if len(matches) == 0 {
		log.Println("no cid was found??")
		return
	}

	var builder strings.Builder
	builder.WriteString("package cid\n\nconst (\n")

	for _, match := range matches {
		name := strings.Replace(match[1], "Cmd", "", 1)
		value := match[2]
		builder.WriteString(fmt.Sprintf("    %s uint16 = %s\n", name, value))
	}

	builder.WriteString(")\n")

	cidDir := filepath.Join(filepath.Dir(filePath), "cid")
	if err := os.MkdirAll(cidDir, 0755); err != nil {
		log.Fatalf("failed to create cid directory: %v", err)
	}

	outputPath := filepath.Join(cidDir, "cid.go")
	if err := os.WriteFile(outputPath, []byte(builder.String()), 0644); err != nil {
		log.Fatalf("failed to write cid.go: %v", err)
	}

	log.Println("cid generation success.")
}

func compileBin(filePath, targetDir string) {
	log.Printf("running compileBin(\"%s\", \"%s\").\n", filePath, targetDir)

	cmd := exec.Command("go", "build", "-o", targetDir, filePath)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Run(); err != nil {
		log.Fatalf("compileBin failed: %v\n", err)
	} else {
		log.Println("compileBin success.")
	}
}

func main() {
	compileProto("pb/StarRail.proto")
	compileBin("cmd/server/main.go", "cmd/server/")
}
