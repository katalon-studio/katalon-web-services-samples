import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

ResponseObject response = WS.sendRequest(findTestObject('API/WebService Demo AUT/API_GE_GetUser'))

def slurper = new JsonSlurper()

def result = slurper.parseText(response.getResponseBodyContent())

//Delete first returned user
response = WS.sendRequest(findTestObject('API/WebService Demo AUT/API_DEL_DeleteUser', [('userId') : result.id[0]]))

//Verify if user is deleted successfully
assert response.getStatusCode() == 200

assert response.getResponseBodyContent() == 'Delete successfully'

@com.kms.katalon.core.annotation.SetUp
def setUp() {
    WebUI.callTestCase(findTestCase('REST/TC_CreateUser'), [('age') : 14, ('gender') : 'MALE'], FailureHandling.STOP_ON_FAILURE)
}

