// Current Date and Time: 2026-03-03 07:01:42

// Web Object Locators for Landing Page elements

import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

WebUI.openBrowser('')

// Sample Object Locators
WebUI.click(findTestObject('Object Repository/DemoWebShop/LandingPage/button_Login'))
WebUI.verifyElementVisible(findTestObject('Object Repository/DemoWebShop/LandingPage/text_Label'))
