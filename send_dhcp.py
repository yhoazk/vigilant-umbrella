from scapy.all import DHCP_am
from scapy.base_classes import Net

dhcp_server = DHCP_am(iface='eth0', domain='example.com',
                      pool=Net('169.254.10.0/24'),
                      network='169.254.10.0/24',
                      gw='169.254.10.0/24',
                      renewal_time=600, lease_time=3600)

dhcp_server()
