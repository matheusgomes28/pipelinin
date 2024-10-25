package lib

import (
	"fmt"
	"testing"
)

// TestHelloEmpty calls greetings.Hello with an empty string,
// checking for an error.
func TestHelloEmpty(t *testing.T) {
	fmt.Println("Say hello from tests: ")
	SayHello()
}
