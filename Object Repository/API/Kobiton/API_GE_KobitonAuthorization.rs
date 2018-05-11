<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API_GE_KobitonAuthorization</name>
   <tag></tag>
   <elementGuidId>f7dbb46a-f614-4146-94eb-b4aaa0943533</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;var headers \u003d {\n  \u0027Authorization\u0027: \u0027Basic a2F0YWxvbi1pbnRlZ3JhdGlvbjpkODYxMTk0Ni04MjM5LTRhZTMtYWY0Yi1jNzllOGVmMmU3N2Q\u003d\u0027,\n  \u0027Accept\u0027:\u0027application/json\u0027\n\n};\n$.ajax({\n  url: \u0027https://api.kobiton.com/v1/sessions\u0027,\n  method: \u0027get\u0027,\n\n  headers: headers,\n  success: function(data) {\n    console.log(JSON.stringify(data));\n  }\n})&quot;,
  &quot;contentType&quot;: &quot;application/javascript&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/javascript</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic a2F0YWxvbi1pbnRlZ3JhdGlvbjpkODYxMTk0Ni04MjM5LTRhZTMtYWY0Yi1jNzllOGVmMmU3N2Q=</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.kobiton.com/v1/apps</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
