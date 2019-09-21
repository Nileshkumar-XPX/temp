<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateEmp - allvariable</name>
   <tag></tag>
   <elementGuidId>eff87420-2b51-400c-89c5-9149428f207b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;name\&quot;:\&quot;${EmpName}\&quot;,\&quot;salary\&quot;:\&quot;${salary}\&quot;,\&quot;age\&quot;:\&quot;${age}\&quot;}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1aba97a8-aeab-4511-932d-6663465813f4</id>
      <masked>false</masked>
      <name>EmpName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9f5f6b1c-6d38-4622-a029-6336eb562aa1</id>
      <masked>false</masked>
      <name>salary</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e9f74e30-dbde-4617-a1ed-9631191c46ce</id>
      <masked>false</masked>
      <name>age</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

assertThat(response.responseBodyContent.contains(GlobalVariable.EmpName))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
