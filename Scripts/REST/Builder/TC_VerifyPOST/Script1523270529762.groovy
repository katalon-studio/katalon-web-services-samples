import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.testobject.RestRequestObjectBuilder
import com.kms.katalon.core.testobject.TestObjectProperty as TestObjectProperty
import com.kms.katalon.core.testobject.UrlEncodedBodyParameter
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
 
def builder = new RestRequestObjectBuilder()
  
'Create a new POST object using builder'
def requestObject = builder
    .withRestRequestMethod("POST")
    .withRestUrl("https://sample-web-service-aut.herokuapp.com/api/users/urlencoded")
    .withHttpHeaders([
        new TestObjectProperty("Content-Type", ConditionType.EQUALS, "application/x-www-form-urlencoded")
    ])
    .withUrlEncodedBodyContent([
        new UrlEncodedBodyParameter("username", "myUsername"),
        new UrlEncodedBodyParameter("password", "myPassword"),
        new UrlEncodedBodyParameter("gender", "MALE"),
        new UrlEncodedBodyParameter("age", "20"),
		new UrlEncodedBodyParameter("avatar", "AAA"),
    ])
    .build()
     
def response = WS.sendRequest(requestObject)
  
assert response.getStatusCode() == 200