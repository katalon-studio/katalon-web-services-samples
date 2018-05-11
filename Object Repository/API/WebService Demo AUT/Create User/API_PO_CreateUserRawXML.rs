<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API_PO_CreateUserRawXML</name>
   <tag></tag>
   <elementGuidId>dfe24cd2-f45e-42eb-bfb9-59c6b1848016</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003c?xml version\u003d\&quot;1.0\&quot;?\u003e\n\u003cUser\u003e\n  \u003cage\u003e${age}\u003c/age\u003e\n  \u003cgender\u003e${gender}\u003c/gender\u003e\n  \u003cpassword\u003estring\u003c/password\u003e\n  \u003cusername\u003estring\u003c/username\u003e\n\u003c/User\u003e\n&quot;,
  &quot;contentType&quot;: &quot;application/xml&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/xml</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://sample-web-service-aut.herokuapp.com/api/users/xml</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
