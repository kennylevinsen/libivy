package main

import "C"

import (
	"unsafe"
	"bytes"
	"strings"

	"robpike.io/ivy/config"
	"robpike.io/ivy/exec"
	"robpike.io/ivy/parse"
	"robpike.io/ivy/run"
	"robpike.io/ivy/scan"
	"robpike.io/ivy/value"
)


//export ivy_eval
func ivy_eval(expr *C.char, buf unsafe.Pointer, buflen C.int, errbuf unsafe.Pointer, errbuflen C.int) C.int {
	var (
		conf    config.Config
		bout    bytes.Buffer
		berr    bytes.Buffer
		context value.Context
	)

	conf.SetFormat("%.12g")
	conf.SetMaxBits(1e9)
	conf.SetMaxDigits(1e4)
	conf.SetOrigin(1)
	conf.SetPrompt("")

	conf.SetOutput(&bout)
	conf.SetErrOutput(&berr)

	context = exec.NewContext(&conf)
	scanner := scan.New(context, "<args>", strings.NewReader(C.GoString(expr)))
	parser := parse.NewParser("<args>", scanner, context)
	run.Run(parser, context, false)

	out := bout.Bytes()
	var idx int
	for idx = 0; idx < int(buflen)-1 && idx < len(out) && out[idx] != '\n'; idx++ {
		*(*byte)(unsafe.Pointer(uintptr(buf)+uintptr(idx))) = out[idx]
	}

	*(*byte)(unsafe.Pointer(uintptr(buf)+uintptr(idx))) = 0

	err := berr.Bytes()
	for idx = 0; idx < int(errbuflen)-1 && idx < len(err) && err[idx] != '\n'; idx++ {
		*(*byte)(unsafe.Pointer(uintptr(errbuf)+uintptr(idx))) = err[idx]
	}
	*(*byte)(unsafe.Pointer(uintptr(errbuf)+uintptr(idx))) = 0

	if len(err) != 0 {
		return C.int(-1)
	} else {
		return C.int(0)
	}
}

func main(){}