Using the couchdb database on melange as a backend while we figure out how to make database persistent here. 

Steps to implement: 
1) Create a service and an for melange. 

The service will not have a selector. The endpoint will be created manually and will point to melange ip address.

Note that the service name much match endpoint name which must match the dns name (which, in this case is melange) 

2) Update fluentd-configmap to point to the melange endpoint. 

3) Check log on pod (fluentd) to make sure data is flowing in.

