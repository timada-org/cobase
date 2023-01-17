package http

import (
	"fmt"
	"net/http"

	"github.com/julienschmidt/httprouter"
)

func getGroups(w http.ResponseWriter, r *http.Request, ps httprouter.Params) {
	fmt.Fprintf(w, "hello, %s!\n", ps.ByName("name"))
}

func createGroup(w http.ResponseWriter, r *http.Request, ps httprouter.Params) {
	fmt.Fprintf(w, "hello, %s!\n", ps.ByName("name"))
}

func configureGroups(router *httprouter.Router) {
	router.GET("/api/groups", getGroups)
	router.POST("/api/groups", createGroup)
}
