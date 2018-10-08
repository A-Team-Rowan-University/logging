# logging

This will be used by the other web development services to log messages. This will include things like starting up, loading config, processing requests, any errors, etc. This is so that we have an idea of what is going on, especially if things start to break.

It is important that logging is fast. It can take some time to open a log file, go to the bottom, add a line, and close it. We want to minimize this, so you probably want to save up log messages for a while, then write them all out. You will also want to minimize the amount of data you are writing to the log files. One way of doing this would be to only write a timestamp and a code to the file. This code can then be looked up to get the actual message later.

You will probably want to implement this in steps. First start by just writing log messages to a file. Then work on queuing messages and writing every so often. Once that works, you can try out minimizing size. Chances are we won't be getting enough requests for performance to be an issue for a while, so you have some time.

