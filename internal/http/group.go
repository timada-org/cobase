package http

import (
	"encoding/json"
	"net/http"

	"github.com/julienschmidt/httprouter"
	gonanoid "github.com/matoous/go-nanoid/v2"
)

type Group struct {
	// The main identifier for the Book. This will be unique.
	ID     string `json:"id"`
	Name   string `json:"name"`
	UserID string `json:"user_id"`
}

type CommandResponse struct {
	ID string `json:"id"`
}

type JsonResponse struct {
	// Reserved field to add some meta information to the API response
	Meta interface{} `json:"meta"`
	Data interface{} `json:"data"`
}

func groupGetAll(w http.ResponseWriter, r *http.Request, ps httprouter.Params) {
	groups := []*Group{}

	if id, err := gonanoid.New(); err == nil {
		groups = append(groups, &Group{ID: id, Name: "My group 1", UserID: "253c1f34-3fe1-4684-9118-c74ea1973bea"})
	}

	if id, err := gonanoid.New(); err == nil {
		groups = append(groups, &Group{ID: id, Name: "My group 2", UserID: "253c1f34-3fe1-4684-9118-c74ea1973bea"})
	}

	if id, err := gonanoid.New(); err == nil {
		groups = append(groups, &Group{ID: id, Name: "My group 3", UserID: "253c1f34-3fe1-4684-9118-c74ea1973bea"})
	}

	response := &JsonResponse{Data: &groups}
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(response); err != nil {
		panic(err)
	}

}

func groupCreate(w http.ResponseWriter, r *http.Request, ps httprouter.Params) {
	id, err := gonanoid.New()
	if err != nil {
		panic(err)
	}

	response := &JsonResponse{Data: &CommandResponse{ID: id}}
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(response); err != nil {
		panic(err)
	}
}

func configureGroups(router *httprouter.Router) {
	router.GET("/api/group/get-all", groupGetAll)
	router.POST("/api/group/create", groupCreate)
}
