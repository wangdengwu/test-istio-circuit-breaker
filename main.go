package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"strings"
	"time"
)

var HostUrl, ok = os.LookupEnv("HOST_URL")

func init() {
	if !ok {
		HostUrl = "https://httpbin.org"
	}
}

func home(w http.ResponseWriter, _ *http.Request) {
	_, _ = fmt.Fprintf(w, "Home Page")
}

func status200(w http.ResponseWriter, _ *http.Request) {
	code, body := getHttpBinWithPath("/status/200")
	w.WriteHeader(code)
	_, _ = fmt.Fprintf(w, string(body))
}

func status503(w http.ResponseWriter, _ *http.Request) {
	code, body := getHttpBinWithPath("/status/503")
	w.WriteHeader(code)
	_, _ = fmt.Fprintf(w, string(body))
}

func headers(w http.ResponseWriter, _ *http.Request) {
	code, body := getHttpBinWithPath("/headers")
	w.WriteHeader(code)
	_, _ = fmt.Fprintf(w, string(body))
}

func ip(w http.ResponseWriter, _ *http.Request) {
	code, body := getHttpBinWithPath("/ip")
	w.WriteHeader(code)
	_, _ = fmt.Fprintf(w, string(body))
}

func anything(w http.ResponseWriter, _ *http.Request) {
	url := HostUrl + "/anything"
	s := strings.NewReader("{\"name\":\"wdw\"}")
	response, _ := http.Post(url, "application/json", s)
	w.WriteHeader(response.StatusCode)
	body, _ := io.ReadAll(response.Body)
	_ = response.Body.Close()
	_, _ = fmt.Fprintf(w, string(body))
}

func getHttpBinWithPath(p string) (code int, body []byte) {
	url := HostUrl + p
	rep, _ := http.Get(url)
	body, _ = io.ReadAll(rep.Body)
	_ = rep.Body.Close()
	return rep.StatusCode, body
}

func main() {
	fmt.Println("start caller")
	http.HandleFunc("/", home)
	http.HandleFunc("/status/200", status200)
	http.HandleFunc("/status/503", status503)
	http.HandleFunc("/headers", headers)
	http.HandleFunc("/ip", ip)
	http.HandleFunc("/anything", anything)
	log.Fatal(http.ListenAndServe(":8080", nil))
}

func callHttpBinDelay1s() {
	url := HostUrl + "/delay/1"
	startTime := time.Now()
	rep, _ := http.Get(url)
	fmt.Printf("calling httpbin with %s , status code: %d , used:%s \n", url, rep.StatusCode, time.Since(startTime))
}

func ticker10Times() {
	ticker := time.NewTicker(time.Second * 1)
	defer ticker.Stop()
	for {
		select {
		case t := <-ticker.C:
			s := t.Second()
			if s == 0 || s == 20 || s == 40 {
				fmt.Println("time is ", t.Format("15:04:05"))
				for i := 0; i < 10; i++ {
					go callHttpBinDelay1s()
				}
			}
		}
	}
}
