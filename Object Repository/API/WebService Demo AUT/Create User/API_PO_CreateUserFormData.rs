<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API_PO_CreateUserFormData</name>
   <tag></tag>
   <elementGuidId>6e23dc6a-a720-4704-bdc4-59b3ba003502</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;username&quot;,
      &quot;value&quot;: &quot;1234&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;23124&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;gender&quot;,
      &quot;value&quot;: &quot;MALE&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;age&quot;,
      &quot;value&quot;: &quot;12&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;avatar&quot;,
      &quot;value&quot;: &quot;Data Files/avatar.png&quot;,
      &quot;type&quot;: &quot;File&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://sample-web-service-aut.herokuapp.com/api/users/form-data </restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
