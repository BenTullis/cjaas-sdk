# coding: utf-8

"""
    Customer Journey as a Service (CJaaS)

    Something amazing, something special - the Customer Journey as a Service (CJaaS) is a core data layer to enable Journeys across products built upon serverless multi-cloud architecture, to be available as a SaaS service for applications inside and outside of Cisco. [**Cisco Experimental - Not For Production Use**]  # noqa: E501

    OpenAPI spec version: 0.5.0
    Contact: cjaas-earlyaccess@cisco.com
    Generated by: https://github.com/swagger-api/swagger-codegen.git
"""

import pprint
import re  # noqa: F401

import six

class ProfileViewBuilderTemplateAttribute(object):
    """NOTE: This class is auto generated by the swagger code generator program.

    Do not edit the class manually.
    """
    """
    Attributes:
      swagger_types (dict): The key is attribute name
                            and the value is attribute type.
      attribute_map (dict): The key is attribute name
                            and the value is json key in definition.
    """
    swagger_types = {
        'version': 'str',
        'event': 'str',
        'metadata': 'str',
        'limit': 'int',
        'display_name': 'str',
        'aggregation_mode': 'str'
    }

    attribute_map = {
        'version': 'version',
        'event': 'event',
        'metadata': 'metadata',
        'limit': 'limit',
        'display_name': 'displayName',
        'aggregation_mode': 'aggregationMode'
    }

    def __init__(self, version=None, event=None, metadata=None, limit=None, display_name=None, aggregation_mode=None):  # noqa: E501
        """ProfileViewBuilderTemplateAttribute - a model defined in Swagger"""  # noqa: E501
        self._version = None
        self._event = None
        self._metadata = None
        self._limit = None
        self._display_name = None
        self._aggregation_mode = None
        self.discriminator = None
        if version is not None:
            self.version = version
        if event is not None:
            self.event = event
        if metadata is not None:
            self.metadata = metadata
        if limit is not None:
            self.limit = limit
        if display_name is not None:
            self.display_name = display_name
        if aggregation_mode is not None:
            self.aggregation_mode = aggregation_mode

    @property
    def version(self):
        """Gets the version of this ProfileViewBuilderTemplateAttribute.  # noqa: E501


        :return: The version of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :rtype: str
        """
        return self._version

    @version.setter
    def version(self, version):
        """Sets the version of this ProfileViewBuilderTemplateAttribute.


        :param version: The version of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :type: str
        """

        self._version = version

    @property
    def event(self):
        """Gets the event of this ProfileViewBuilderTemplateAttribute.  # noqa: E501


        :return: The event of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :rtype: str
        """
        return self._event

    @event.setter
    def event(self, event):
        """Sets the event of this ProfileViewBuilderTemplateAttribute.


        :param event: The event of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :type: str
        """

        self._event = event

    @property
    def metadata(self):
        """Gets the metadata of this ProfileViewBuilderTemplateAttribute.  # noqa: E501


        :return: The metadata of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :rtype: str
        """
        return self._metadata

    @metadata.setter
    def metadata(self, metadata):
        """Sets the metadata of this ProfileViewBuilderTemplateAttribute.


        :param metadata: The metadata of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :type: str
        """

        self._metadata = metadata

    @property
    def limit(self):
        """Gets the limit of this ProfileViewBuilderTemplateAttribute.  # noqa: E501


        :return: The limit of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :rtype: int
        """
        return self._limit

    @limit.setter
    def limit(self, limit):
        """Sets the limit of this ProfileViewBuilderTemplateAttribute.


        :param limit: The limit of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :type: int
        """

        self._limit = limit

    @property
    def display_name(self):
        """Gets the display_name of this ProfileViewBuilderTemplateAttribute.  # noqa: E501


        :return: The display_name of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :rtype: str
        """
        return self._display_name

    @display_name.setter
    def display_name(self, display_name):
        """Sets the display_name of this ProfileViewBuilderTemplateAttribute.


        :param display_name: The display_name of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :type: str
        """

        self._display_name = display_name

    @property
    def aggregation_mode(self):
        """Gets the aggregation_mode of this ProfileViewBuilderTemplateAttribute.  # noqa: E501


        :return: The aggregation_mode of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :rtype: str
        """
        return self._aggregation_mode

    @aggregation_mode.setter
    def aggregation_mode(self, aggregation_mode):
        """Sets the aggregation_mode of this ProfileViewBuilderTemplateAttribute.


        :param aggregation_mode: The aggregation_mode of this ProfileViewBuilderTemplateAttribute.  # noqa: E501
        :type: str
        """

        self._aggregation_mode = aggregation_mode

    def to_dict(self):
        """Returns the model properties as a dict"""
        result = {}

        for attr, _ in six.iteritems(self.swagger_types):
            value = getattr(self, attr)
            if isinstance(value, list):
                result[attr] = list(map(
                    lambda x: x.to_dict() if hasattr(x, "to_dict") else x,
                    value
                ))
            elif hasattr(value, "to_dict"):
                result[attr] = value.to_dict()
            elif isinstance(value, dict):
                result[attr] = dict(map(
                    lambda item: (item[0], item[1].to_dict())
                    if hasattr(item[1], "to_dict") else item,
                    value.items()
                ))
            else:
                result[attr] = value
        if issubclass(ProfileViewBuilderTemplateAttribute, dict):
            for key, value in self.items():
                result[key] = value

        return result

    def to_str(self):
        """Returns the string representation of the model"""
        return pprint.pformat(self.to_dict())

    def __repr__(self):
        """For `print` and `pprint`"""
        return self.to_str()

    def __eq__(self, other):
        """Returns true if both objects are equal"""
        if not isinstance(other, ProfileViewBuilderTemplateAttribute):
            return False

        return self.__dict__ == other.__dict__

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        return not self == other
