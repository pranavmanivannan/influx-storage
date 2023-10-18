# TO-DO

1) Figure out best way to send data over channel (deserialize or just send raw struct, dont send string)
2) Remove Data trait for filtering in Receiver/Sender so that we don't have runtime polymorphism and find
    a different way to filter for compile-time
3) Get AWS uploading through HTTP requests sorted out
4) Make unit tests