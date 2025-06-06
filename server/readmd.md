aarch64-unknown-linux-gnu
t4g.nano
vcpu: 2
mem: 0.5

```
ssh -i "Ec2TetrisServer.pem" ec2-user@${PUBLIC_DNS}
```

```
sudo dnf install -y git
sudo dnf install -y nginx
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


```
sudo systemctl start nginx
# sudo systemctl status nginx

```

