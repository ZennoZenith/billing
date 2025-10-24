POST http://localhost:8080/api/seller
[{
  "name": "sel {{newUuid}}"
}, {
  "name": "sel {{newUuid}}"
}]
HTTP 200
[Captures]
seller_id: jsonpath "$.result.sellers[0].sellerId" 


POST http://localhost:8080/api/bill
{
  "remark": "Some bill remark",
  "sellerId": "{{seller_id}}"
}
HTTP 200
[Captures]
bill_id: jsonpath "$.result.billId" 


POST http://localhost:8080/api/transaction

[{
  "name": "tr 1",
  "transactionTime": null,
  "remark": null,
  "tags": ["a", "b "],
  "paymentMethod": "Cash",
  "unitCost": {
    "unitType": "kg",
    "unit": 1.4,
    "costPerUnit": 58.0
  },
  "sellerCost": 10.4,
  "billId": null
}, {
  "name": "tr 2",
  "transactionTime": "2020-09-08T13:10:08.511Z",
  "remark": "Some remark",
  "tags": ["d", "c"],
  "paymentMethod": "Upi",
  "unitCost": {
    "unitType": "bag",
    "unit": 5,
    "costPerUnit": 340
  },
  "sellerCost": 1700,
  "billId": "{{bill_id}}"
}]
