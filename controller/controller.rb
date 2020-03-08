require 'socket'

def respond(info, socket)
  message, opts = info
  aftype, port, remote, local = opts
  socket.send message + "drop", 0, remote, port
end

u = UDPSocket.new
u.bind("0.0.0.0", 1234)
loop do
  info = pp u.recvfrom 1500
  respond info, u
end
