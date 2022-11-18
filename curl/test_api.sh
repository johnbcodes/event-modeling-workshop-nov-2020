#!/bin/bash

echo "Making reservation"
curl -i --location --request POST 'http://localhost:3030/reservation/make/reservation-0001'\
  --header 'Content-Type: application/json'\
  --data "@MakeReservation.json"
echo
echo "Retrieving reservation"
curl -i --location 'http://localhost:3030/reservation/reservation-0001'
echo
echo "Cancelling reservation"
curl -i --location --request POST 'http://localhost:3030/reservation/cancel/reservation-0001'\
  --header 'Content-Type: application/json'
echo
