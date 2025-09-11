package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"time"

	expect "github.com/google/goexpect"
	"golang.org/x/crypto/ssh"
)

func main() {
	if len(os.Args) != 7 {
		fmt.Printf("Usage: %s <node_ip> <cpe_ip> <username> <password> <mode> <command>\n", os.Args[0])
		os.Exit(1)
	}

	nodeIP := os.Args[1]
	cpeIP := os.Args[2]
	username := os.Args[3]
	password := os.Args[4]
	mode := os.Args[5]
	command := os.Args[6]

	xPwd := "XnM"
	oPwd := "OnM"

	sshConfig := &ssh.ClientConfig{
		User:            username,
		Auth:            []ssh.AuthMethod{ssh.Password(password)},
		HostKeyCallback: ssh.InsecureIgnoreHostKey(),
		Timeout:         5 * time.Second,
	}

	// ---------- 1. 登录跳板机 ----------
	conn, err := ssh.Dial("tcp", "master.jump.7x-networks.net:2222", sshConfig)
	if err != nil {
		log.Fatal("Failed to dial jump host:", err)
	}
	defer conn.Close()

	e, _, err := expect.SpawnSSH(conn, 2*time.Second, expect.Verbose(true))
	if err != nil {
		log.Fatal(err)
	}
	defer e.Close()

	// 处理 yes/no 和密码
	if _, err := e.ExpectBatch([]expect.Batcher{
		&expect.BExp{R: "yes/no|assword:"},
	}, 5*time.Second); err == nil {
		e.Send("yes\n")
		e.Expect(regexp.MustCompile("assword:"), 5*time.Second)
		e.Send(password + "\n")
	}

	e.Expect(regexp.MustCompile("Opt>"), 5*time.Second)
	e.Send(nodeIP + "\n")
	e.Expect(regexp.MustCompile("seven@.*~\\]$"), 5*time.Second)
	e.Send("su -\n")
	e.Expect(regexp.MustCompile("assword:"), 5*time.Second)
	e.Send(xPwd + "\n")
	e.Expect(regexp.MustCompile("root@.*~\\]#"), 5*time.Second)

	// ---------- 2. SSH 到目标 CPE ----------
	var sshCmd string
	if mode != "ucpe" {
		sshCmd = fmt.Sprintf("ssh -i /etc/openvpn/server/box seven@%s -p 7722", cpeIP)
	} else {
		sshCmd = fmt.Sprintf("ssh -i /etc/openvpn/server/box root@%s -p 7722", cpeIP)
	}

	e.Send(sshCmd + "\n")
	if _, err := e.ExpectBatch([]expect.Batcher{
		&expect.BExp{R: "yes/no|Enter passphrase"},
	}, 5*time.Second); err == nil {
		e.Send("yes\n")
		e.Expect(regexp.MustCompile("Enter passphrase for key '/etc/openvpn/server/box':"), 5*time.Second)
		e.Send(oPwd + "\n")
	}

	e.Expect(regexp.MustCompile("\\$|#"), 5*time.Second)
	if mode != "ucpe" {
		e.Send("su -\n")
		e.Expect(regexp.MustCompile("assword:"), 5*time.Second)
		e.Send(xPwd + "\n")
		e.Expect(regexp.MustCompile("#"), 5*time.Second)
	}

	// ---------- 3. 执行动态命令 ----------
	e.Send(command + "\n")

	// ---------- 4. 交互模式，exit/quit退出 ----------
	fmt.Println("✅ Entering interactive mode. Type 'exit' or 'quit' to leave.")
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		input := scanner.Text()
		if input == "exit" || input == "quit" {
			break
		}
		e.Send(input + "\n")
		e.Expect(regexp.MustCompile("\\$|#"), 5*time.Second)
	}

	e.Send("exit\n")
	fmt.Println("Session closed")
}
