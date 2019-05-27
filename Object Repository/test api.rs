<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>test api</name>
   <tag></tag>
   <elementGuidId>5ee9a8ec-f278-4da4-ad93-cf8404cf02a4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/soap+xml</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Charset</name>
      <type>Main</type>
      <value>utf-8</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic U3VzYW46TWVjQDEyMTI=</value>
   </httpHeaderProperties>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;SOAP:Envelope xmlns:SOAP=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
  &lt;SOAP:Body>
    &lt;CalculateJobDueDate xmlns=&quot;http://schemas.monster.com/jobrequest/1.0&quot; preserveSpace=&quot;no&quot; qAccess=&quot;0&quot; qValues=&quot;&quot;>
      &lt;applicationId>MPM-RM&lt;/applicationId>
      &lt;jobId>1163504&lt;/jobId>
    &lt;/CalculateJobDueDate>
  &lt;/SOAP:Body>
&lt;/SOAP:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>POST</soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress>http://mmqa.monsterenergy.com/bpm/MPM/com.eibus.web.soap.Gateway.wcp?test_ct=da8a37ab28eef97f3f442ea13836b6686951306f</wsdlAddress>
</WebServiceRequestEntity>
