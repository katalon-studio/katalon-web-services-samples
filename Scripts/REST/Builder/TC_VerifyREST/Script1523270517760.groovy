import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.testobject.RestRequestObjectBuilder
import com.kms.katalon.core.testobject.TestObjectProperty as TestObjectProperty
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
 
'Create a new GET object using builder'
def builder = new RestRequestObjectBuilder()
def requestObject = builder
    .withRestRequestMethod("GET")
    .withRestUrl("http://jsonplaceholder.typicode.com/comments")
    .withRestParameters([
        new TestObjectProperty("postId", ConditionType.EQUALS, "1"),
        new TestObjectProperty("id", ConditionType.EQUALS, "1")
    ])
    .withHttpHeaders([
        new TestObjectProperty("Content-Type", ConditionType.EQUALS, "application/json")
    ])
    .build()
	
'Send a request'
def response = WS.sendRequest(requestObject)
 
'Verify if comment\'s email after sending request is correct or not'
WS.verifyElementPropertyValue(response, '[0].email', 'Eliseo@gardner.biz')
