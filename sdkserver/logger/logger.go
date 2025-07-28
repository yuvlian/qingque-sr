package logger

import (
	"fmt"
	"net/http"
)

func Middleware(next http.Handler, addr string) http.Handler {
	fmt.Println("sdkserver listening on:", addr)
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Printf("%s %s\n", r.Method, r.URL.Path)
		next.ServeHTTP(w, r)
	})
}
