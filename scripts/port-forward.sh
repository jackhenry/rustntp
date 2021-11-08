sudo iptables -t nat -A PREROUTING -p udp -i enp6s0 -d 127.0.0.1 --dport 123 -j DNAT --to-destination 127.0.0.1:9002
sudo iptables -A FORWARD -i enp6s0 -p udp -d 127.0.0.1 --dport 9002 -m state --state NEW,ESTABLISHED,RELATED -j ACCEPT
iptables -t nat -A PREROUTING -i eth0 -d 192.168.1.2 -p udp --dport 1003 -j REDIRECT --to-ports 1004

sudo iptables -t nat -I OUTPUT -p udp -o lo --dport 123 -j REDIRECT --to-ports 9002