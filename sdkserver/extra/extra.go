package extra

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/yuvlian/qingque-sr/config"
	"github.com/yuvlian/qingque-sr/config/lx"
	"github.com/yuvlian/qingque-sr/config/srtools"
)

const (
	luaFilesDir string = "./luas/"
	srtFilesDir string = "./srtcfgs/"
)

func AddHandlers(h *http.ServeMux) {
	lxRoute, srtmRoute := "/lua_executor", "/srtools_manager"
	fmt.Printf("sdkserver extras:\n - lua executor at %s\n - srtools manager at %s\n", lxRoute, srtmRoute)
	h.HandleFunc(lxRoute, luaExecutorHandler)
	h.HandleFunc(srtmRoute, srToolsManagerHandler)
}

func luaExecutorHandler(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		w.Header().Set("Content-Type", "text/html")
		w.Write([]byte(luaExecutorPage))

	case http.MethodPost:
		data := r.FormValue("data")
		parts := strings.Split(data, "###")

		if strings.HasPrefix(data, "SAVE###") && len(parts) == 3 {
			filename := filepath.Base(parts[1])
			content := parts[2]

			if !strings.HasSuffix(filename, ".lua") {
				filename += ".lua"
			}

			err := os.MkdirAll(luaFilesDir, 0755)
			if err != nil {
				http.Error(w, "Failed to create directory", 500)
				return
			}

			err = os.WriteFile(luaFilesDir+filename, []byte(content), 0644)
			if err != nil {
				http.Error(w, "Failed to save file", 500)
				return
			}

			w.Write([]byte("Saved as " + filename))
		} else if strings.HasPrefix(data, "EXECUTE###") && len(parts) == 2 {
			content := parts[1]
			lxcfg := lx.Lx{
				ShouldExec: true,
				LuaContent: content,
			}

			err := lxcfg.Save(config.LxFilePath)
			if err != nil {
				http.Error(w, "Failed to execute", 500)
				return
			}

			w.Write([]byte("Lua script sent for execution"))
		} else {
			http.Error(w, "Invalid request format", http.StatusBadRequest)
		}

	default:
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	}
}

func srToolsManagerHandler(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		w.Header().Set("Content-Type", "text/html")
		w.Write([]byte(srToolsManagerPage))

	case http.MethodPost:
		data := r.FormValue("data")
		parts := strings.Split(data, "###")

		if strings.HasPrefix(data, "SAVE###") && len(parts) == 3 {
			filename := filepath.Base(parts[1])
			content := parts[2]

			if !strings.HasSuffix(filename, ".json") {
				filename += ".json"
			}

			cfg, err := srtools.LoadFromString(content)
			if err != nil {
				http.Error(w, err.Error(), http.StatusBadRequest)
				return
			}

			err = cfg.Save(srtFilesDir + filename)
			if err != nil {
				http.Error(w, "Failed to save file", http.StatusInternalServerError)
				return
			}

			w.Write([]byte("Saved as " + filename))
		} else if strings.HasPrefix(data, "LOAD###") && len(parts) == 2 {
			content := parts[1]

			cfg, err := srtools.LoadFromString(content)
			if err != nil {
				http.Error(w, err.Error(), http.StatusBadRequest)
				return
			}

			err = cfg.Save(config.SRToolsLiteFilePath)
			if err != nil {
				http.Error(w, "Failed to load into config", http.StatusInternalServerError)
				return
			}

			w.Write([]byte("Loaded into " + config.SRToolsLiteFilePath))
		} else {
			http.Error(w, "Invalid request format", http.StatusBadRequest)
		}

	default:
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	}
}
