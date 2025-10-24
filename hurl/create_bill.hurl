POST http://localhost:8080/api/seller
[{
  "name": "sel {{newUuid}}"
}, {
  "name": "sel {{newUuid}}"
}]
HTTP 200
[Captures]
seller_id: jsonpath "$['result'].['sellers'][0].['sellerId']" 


POST http://localhost:8080/api/bill
{
  "remark": "Some bill remark",
  "sellerId": "{{seller_id}}"
}


