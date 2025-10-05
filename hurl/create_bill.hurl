POST http://localhost:8080/seller
[{
  "name": "sel {{newUuid}}"
}, {
  "name": "sel {{newUuid}}"
}]
HTTP 200
[Captures]
seller_id: jsonpath "$['result'].['sellers'][0].['sellerId']" 


POST http://localhost:8080/bill
{
  "remark": "Some bill remark",
  "sellerId": "{{seller_id}}"
}
[Captures]
bill_id: jsonpath "$['result'].['billId']" 


