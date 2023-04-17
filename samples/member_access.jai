lol :: () {
    // I did not know what else to use for "member access", so I chose the OS enum.
    #if OS == .LINUX {
        print("Linux rocks!\n");
    }
    #if OS == .MACOS {
        print("MacOS sucks!\n");
    }
    #if OS == .WINDOWS {
        print("No comment on that...\n");
    }
}