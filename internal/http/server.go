package http

import (
	"fmt"
	"log"
	"net/http"

	"github.com/julienschmidt/httprouter"
)

func Hello(w http.ResponseWriter, r *http.Request, ps httprouter.Params) {
	fmt.Fprintf(w, "hello, %s!\n", ps.ByName("name"))
}

type ServerOptions struct {
	StaticPath string
	Addr       string
}

type Server struct {
	options ServerOptions
}

func (s *Server) Start() {
	router := httprouter.New()

	router.GET("/hello/:name", Hello)

	if s.options.StaticPath != "" {
		fileServer := http.FileServer(http.Dir(s.options.StaticPath))

		router.GET("/static/*filepath", func(w http.ResponseWriter, req *http.Request, ps httprouter.Params) {
			req.URL.Path = ps.ByName("filepath")
			fileServer.ServeHTTP(w, req)
		})

		router.NotFound = http.HandlerFunc(func(w http.ResponseWriter, req *http.Request) {
			w.Header().Add("Expires", "Tue, 03 Jul 2001 06:00:00 GMT")

			// w.Header().Add("Last-Modified", "Tue, 03 Jul 2001 06:00:00 GMT")
			w.Header().Add("Cache-Control", "max-age=0, no-cache, must-revalidate, proxy-revalidate")
			http.ServeFile(w, req, s.options.StaticPath+"/index.html")
		})
	}

	log.Fatal(http.ListenAndServe(s.options.Addr, router))
}

func NewServer(options ServerOptions) Server {
	if options.Addr == "" {
		log.Fatal("[server] addr is required")
	}

	return Server{
		options,
	}
}
