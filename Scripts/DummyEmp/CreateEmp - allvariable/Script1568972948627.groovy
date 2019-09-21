import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper


def now =new Date()

now.format("yyyyMMdd-HH:mm:ss.SSS", TimeZone.getTimeZone('UTC'))

GlobalVariable.EmpName= now.toString()
response = WS.sendRequest(findTestObject('DummyEmp/CreateEmp - allvariable',['EmpName':now.toString(), 'salary': salary,'age': age]))

//response = WS.sendRequestAndVerify(findTestObject('DummyEmp/CreateEmp'))
println response.getResponseBodyContent()
String responseText=response.getResponseBodyContent()
println responseText
JsonSlurper slurper =new JsonSlurper()
Map parsedJson =slurper.parseText(responseText)

String id=parsedJson.get("id")

println id 

println GlobalVariable.EmpId=id

assert WS.containsString(response, now.toString(), false)

WS.verifyElementPropertyValue(response,'name',GlobalVariable.EmpName)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('DummyEmp/GetEmployee'))
println response.getResponseBodyContent()
WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('DummyEmp/UpdateEmp',['EmpName':'easy4', 'salary': '150','age': '55']))
WS.verifyElementPropertyValue(response,'salary','150')
String responseText1=response.getResponseBodyContent()
println responseText1
JsonSlurper slurper1 =new JsonSlurper()
Map parsedJson1 =slurper1.parseText(responseText)
String id1=parsedJson1.get("id")
println GlobalVariable.EmpId=id1

response = WS.sendRequest(findTestObject('DummyEmp/DeleteEmp'))
println response.getResponseBodyContent()
WS.verifyResponseStatusCode(response, 200)