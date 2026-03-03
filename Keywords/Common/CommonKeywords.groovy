// CommonKeywords.groovy

package Keywords.Common

class CommonKeywords {

    // Function to open a URL
    static void openUrl(String url) {
        WebUI.openUrl(url)
    }

    // Function to click on an element
    static void clickElement(TestObject testObject) {
        WebUI.click(testObject)
    }

    // Function to verify text
    static boolean verifyText(String expectedText, TestObject testObject) {
        return WebUI.verifyText(expectedText, testObject)
    }

}