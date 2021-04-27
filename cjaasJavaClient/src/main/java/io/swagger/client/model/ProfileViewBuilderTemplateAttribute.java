/*
 * Customer Journey as a Service (CJaaS)
 * Something amazing, something special - the Customer Journey as a Service (CJaaS) is a core data layer to enable Journeys across products built upon serverless multi-cloud architecture, to be available as a SaaS service for applications inside and outside of Cisco. [**Cisco Experimental - Not For Production Use**]
 *
 * OpenAPI spec version: 0.5.0
 * Contact: cjaas-earlyaccess@cisco.com
 *
 * NOTE: This class is auto generated by the swagger code generator program.
 * https://github.com/swagger-api/swagger-codegen.git
 * Do not edit the class manually.
 */

package io.swagger.client.model;

import java.util.Objects;
import java.util.Arrays;
import com.google.gson.TypeAdapter;
import com.google.gson.annotations.JsonAdapter;
import com.google.gson.annotations.SerializedName;
import com.google.gson.stream.JsonReader;
import com.google.gson.stream.JsonWriter;
import io.swagger.v3.oas.annotations.media.Schema;
import java.io.IOException;
/**
 * ProfileViewBuilderTemplateAttribute
 */

@javax.annotation.Generated(value = "io.swagger.codegen.v3.generators.java.JavaClientCodegen", date = "2021-02-10T15:46:40.647328-07:00[America/Denver]")
public class ProfileViewBuilderTemplateAttribute {
  @SerializedName("version")
  private String version = null;

  @SerializedName("event")
  private String event = null;

  @SerializedName("metadata")
  private String metadata = null;

  @SerializedName("limit")
  private Integer limit = null;

  @SerializedName("displayName")
  private String displayName = null;

  @SerializedName("aggregationMode")
  private String aggregationMode = null;

  public ProfileViewBuilderTemplateAttribute version(String version) {
    this.version = version;
    return this;
  }

   /**
   * Get version
   * @return version
  **/
  @Schema(description = "")
  public String getVersion() {
    return version;
  }

  public void setVersion(String version) {
    this.version = version;
  }

  public ProfileViewBuilderTemplateAttribute event(String event) {
    this.event = event;
    return this;
  }

   /**
   * Get event
   * @return event
  **/
  @Schema(description = "")
  public String getEvent() {
    return event;
  }

  public void setEvent(String event) {
    this.event = event;
  }

  public ProfileViewBuilderTemplateAttribute metadata(String metadata) {
    this.metadata = metadata;
    return this;
  }

   /**
   * Get metadata
   * @return metadata
  **/
  @Schema(description = "")
  public String getMetadata() {
    return metadata;
  }

  public void setMetadata(String metadata) {
    this.metadata = metadata;
  }

  public ProfileViewBuilderTemplateAttribute limit(Integer limit) {
    this.limit = limit;
    return this;
  }

   /**
   * Get limit
   * @return limit
  **/
  @Schema(description = "")
  public Integer getLimit() {
    return limit;
  }

  public void setLimit(Integer limit) {
    this.limit = limit;
  }

  public ProfileViewBuilderTemplateAttribute displayName(String displayName) {
    this.displayName = displayName;
    return this;
  }

   /**
   * Get displayName
   * @return displayName
  **/
  @Schema(description = "")
  public String getDisplayName() {
    return displayName;
  }

  public void setDisplayName(String displayName) {
    this.displayName = displayName;
  }

  public ProfileViewBuilderTemplateAttribute aggregationMode(String aggregationMode) {
    this.aggregationMode = aggregationMode;
    return this;
  }

   /**
   * Get aggregationMode
   * @return aggregationMode
  **/
  @Schema(description = "")
  public String getAggregationMode() {
    return aggregationMode;
  }

  public void setAggregationMode(String aggregationMode) {
    this.aggregationMode = aggregationMode;
  }


  @Override
  public boolean equals(java.lang.Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ProfileViewBuilderTemplateAttribute profileViewBuilderTemplateAttribute = (ProfileViewBuilderTemplateAttribute) o;
    return Objects.equals(this.version, profileViewBuilderTemplateAttribute.version) &&
        Objects.equals(this.event, profileViewBuilderTemplateAttribute.event) &&
        Objects.equals(this.metadata, profileViewBuilderTemplateAttribute.metadata) &&
        Objects.equals(this.limit, profileViewBuilderTemplateAttribute.limit) &&
        Objects.equals(this.displayName, profileViewBuilderTemplateAttribute.displayName) &&
        Objects.equals(this.aggregationMode, profileViewBuilderTemplateAttribute.aggregationMode);
  }

  @Override
  public int hashCode() {
    return Objects.hash(version, event, metadata, limit, displayName, aggregationMode);
  }


  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ProfileViewBuilderTemplateAttribute {\n");
    
    sb.append("    version: ").append(toIndentedString(version)).append("\n");
    sb.append("    event: ").append(toIndentedString(event)).append("\n");
    sb.append("    metadata: ").append(toIndentedString(metadata)).append("\n");
    sb.append("    limit: ").append(toIndentedString(limit)).append("\n");
    sb.append("    displayName: ").append(toIndentedString(displayName)).append("\n");
    sb.append("    aggregationMode: ").append(toIndentedString(aggregationMode)).append("\n");
    sb.append("}");
    return sb.toString();
  }

  /**
   * Convert the given object to string with each line indented by 4 spaces
   * (except the first line).
   */
  private String toIndentedString(java.lang.Object o) {
    if (o == null) {
      return "null";
    }
    return o.toString().replace("\n", "\n    ");
  }

}
