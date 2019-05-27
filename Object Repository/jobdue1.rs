<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>jobdue1</name>
   <tag></tag>
   <elementGuidId>d959593f-ea91-41a5-ab6b-94b6d3a16559</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SAMLart</name>
      <type>Main</type>
      <value>MDG+YQC6e2EELaGB1sxcxMV++4Ptdc3jIF7nf9Dd2Bzi4Fx6WWlWg2bv	</value>
   </httpHeaderProperties>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;SOAP:Envelope xmlns:SOAP='http://schemas.xmlsoap.org/soap/envelope/'>&lt;SOAP:Body>&lt;CalculateJobDueDate xmlns='http://schemas.monster.com/jobrequest/1.0' >&lt;applicationId>MPM-RM&lt;/applicationId>&lt;jobId>1277960&lt;/jobId>&lt;/CalculateJobDueDate>&lt;/SOAP:Body>&lt;/SOAP:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>CalculateJobDueDate</soapServiceFunction>
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
   <wsdlAddress>https://mmqa.monsterenergy.com/cordys/WSDLGateway.wcp?service=http%3A%2F%2Fschemas.monster.com%2Fjobrequest%2F1.0/CalculateJobDueDate&amp;organization=o%3DMPM%2Ccn%3Dcordys%2Ccn%3DTEST%2Co%3DHBC.internal&amp;version=isv</wsdlAddress>
</WebServiceRequestEntity>
