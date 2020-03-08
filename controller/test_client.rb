require 'socket'

host, port = "127.0.0.1", 1234

command = <<-EOS
4
6
127001
1235
127002
1236
EOS

client = UDPSocket.new
client.send command, 0, host, port

m = client.recvfrom 1500
message, opts = m

puts message
