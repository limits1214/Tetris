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

```
server {
    listen 80;
    server_name _;

    location / {
        proxy_pass http://127.0.0.1:4000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```


```
sudo nginx -t                # 설정 확인 (꼭)
sudo systemctl reload nginx  # 설정 반영
```