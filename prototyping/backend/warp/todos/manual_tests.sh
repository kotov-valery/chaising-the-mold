#!/usr/bin/bash

echo "Request a list of todos. Should be empty"
curl -X GET localhost:3030/todos
echo ""

echo "Add a todo item to the list"
curl -X POST\
  -H "Content-type: application/json" \
  -H "Accept: application/json" \
  -d '{"id":68, "description": "A simple todo item", "completed": false}' \
  localhost:3030/todos

echo "Request a list of todos. Now there should be one"
curl -X GET localhost:3030/todos
echo ""

echo "Add another item to the list"
curl -X POST\
  -H "Content-type: application/json" \
  -H "Accept: application/json" \
  -d '{"id":15, "description": "Another interesting item in the list", "completed": false}' \
  localhost:3030/todos

echo "Request a list of todos. Now there should be two"
curl -X GET localhost:3030/todos
echo ""

echo "Now update first item as completed"
curl -X PUT\
  -H "Content-type: application/json" \
  -H "Accept: application/json" \
  -d '{"id":68, "description": "A simple todo item", "completed": true}' \
  localhost:3030/todos/68

echo "Request a list of todos. Now first item should be completed and second one should remain the same"
curl -X GET localhost:3030/todos
echo ""

echo "Now delete the first item"
curl -X DELETE localhost:3030/todos/68

echo "And check if it was deleted"
curl -X GET localhost:3030/todos
echo ""


