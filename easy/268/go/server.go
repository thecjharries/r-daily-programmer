package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func bootstrapApp() *gin.Engine {
	engine := gin.Default()
	engine.GET("/ping", endpointPing)
	return engine
}

func endpointPing(context *gin.Context) {
	context.JSON(
		http.StatusOK,
		gin.H{
			"message": "pong",
		},
	)
}
