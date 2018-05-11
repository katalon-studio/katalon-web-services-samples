<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API_PO_CreateUserUrlEncoded</name>
   <tag></tag>
   <elementGuidId>4941bd32-4520-4cc6-839d-071573db834b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;username&quot;,
      &quot;value&quot;: &quot;321&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;432432&quot;
    },
    {
      &quot;name&quot;: &quot;gender&quot;,
      &quot;value&quot;: &quot;${gender}&quot;
    },
    {
      &quot;name&quot;: &quot;age&quot;,
      &quot;value&quot;: &quot;${age}&quot;
    },
    {
      &quot;name&quot;: &quot;avatar&quot;,
      &quot;value&quot;: &quot;Data Files/avatar.png&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://sample-web-service-aut.herokuapp.com/api/users/urlencoded</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
